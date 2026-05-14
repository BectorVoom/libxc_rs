//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1294/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1294<F: Float>(t100915: F, t101282: F, t104845: F, t104851: F, t104857: F, t104860: F, t104868: F, t104878: F, t104884: F, t1355: F, t22767: F, t23701: F, t23711: F, t23732: F, t23774: F, t26635: F, t26639: F, t26643: F, t26665: F, t26678: F, t26729: F, t5579: F, t61777: F, t72: F, t94613: F, t94936: F) -> (F,) {
    let t104885 = 0.56502890877451119022e-1 * t94613 * t104845 + 0.90613700826057446696e0 * t26729 * t26678 + 0.76518236253115177207e1 * t1355 * t104851 - 0.48327307107230638238e1 * t26729 * t26665 - 0.24163653553615319118e1 * t1355 * t104857 - 0.48335523541469733928e0 * t104860 * t26635 - 0.10668480174814814815e1 * t23732 * t22767 * t26639 + t104868 + 0.12002040196666666667e1 * t94936 * t5579 * t72 * t61777 + 0.16002720262222222222e1 * t23774 * t22767 * t26643 - t104878 + 0.1611184118048991131e0 * t23711 * t100915 + 0.46992870109762241322e0 * t23701 * t101282 - t104884;
    (t104885,)
}
