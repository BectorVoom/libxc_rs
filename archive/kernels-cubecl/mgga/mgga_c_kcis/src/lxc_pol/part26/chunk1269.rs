//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1269/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1269<F: Float>(t28884: F, t28887: F, t28889: F, t28891: F, t27741: F, t6290: F, t7671: F, t20853: F, t2167: F, t97601: F, t26657: F, t29238: F, t29249: F, t29251: F, t29253: F, t29256: F, t29660: F, t91769: F, t91772: F, t91773: F, t91776: F, t91777: F, t91778: F, t91781: F, t95271: F, t97622: F, t97623: F, t97624: F, t97625: F) -> (F, F, F, F, F, F, F, F) {
    let t99793 = t28884 / F::cast_from(8.0_f64);
    let t99794 = t28887 / F::cast_from(8.0_f64);
    let t99795 = t28889 / F::cast_from(8.0_f64);
    let t99796 = t28891 / F::cast_from(8.0_f64);
    let t99798 = F::cast_from(4.0_f64) * t27741;
    let t99834 = t6290 * t7671;
    let t101750 = t20853 * t2167;
    let t101757 = F::cast_from(2.0_f64) * t97601;
    let t101774 = -t29249 - t91769 + t91772 + t29251 + t29238 + t91773 - t29253 + t97622 + t95271 - t91776 - t97623 + t91777 + t99798 - t91778 + t26657 - t29256 - t29660 - t97624 - t91781 + t97625;
    (t99793, t99794, t99795, t99796, t99834, t101750, t101757, t101774)
}
