//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1183/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1183<F: Float>(t24330: F, t28587: F, t6249: F, t111881: F, t111889: F, t111892: F, t111895: F, t111897: F, t19072: F, t25112: F, t25113: F, t25123: F, t25127: F, t27506: F, t6242: F, t6821: F, t684: F, t7013: F, t98434: F, t98520: F, t98545: F) -> (F,) {
    let t111901 = t6249 * t24330 * t28587;
    let t111906 = -0.20003400327777777778e0 * t98520 * t98545 * t19072 * t684 + 0.20003400327777777778e0 * t98434 * t7013 + 0.22226000364197530864e-1 * t111881 + 0.26671200437037037038e0 * t6242 * t27506 * t25123 - 0.26671200437037037038e0 * t6249 * t27506 * t25127 - 0.66678001092592592594e-1 * t111889 - 0.66678001092592592594e-1 * t111892 - 0.22226000364197530865e-1 * t111895 - 0.24167761770734866966e0 * t111897 * t6821 + 0.66678001092592592594e-1 * t111901 + 0.80013601311111111114e0 * t25112 * t27506 * t25113;
    (t111906,)
}
