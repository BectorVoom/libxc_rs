//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 970/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk970<F: Float>(t191: F, t192: F, t5118: F, t2020: F, t6997: F, t7685: F, t1390: F, t5187: F, t6878: F, t1983: F, t531: F, t1982: F) -> (F, F, F, F, F, F, F, F) {
    let t24987 = t5118 * t191 * t192;
    let t24988 = t24987 * t2020;
    let t24989 = t7685 * t6997;
    let t24990 = t1390 * t5187;
    let t24991 = t6878 * t24990;
    let t24993 = F::cast_from(3.0_f64) * t1983 * t24991;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    (t24987, t24988, t24989, t24990, t24991, t24993, t24994, t24995)
}
