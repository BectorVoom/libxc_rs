//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1241/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1241<F: Float>(t2036: F, t2040: F, t2075: F, t2114: F, t2165: F, t27863: F, t32674: F, t33345: F, t33360: F, t33361: F, t33364: F, t33365: F, t33367: F, t33690: F, t7266: F, t7787: F, t7796: F, t7890: F, t7983: F, t8103: F) -> F {
    let t34115 = -t2036 * t8103 - F::cast_from(2.0_f64) * t2040 * t27863 - F::cast_from(2.0_f64) * t2040 * t33690 - t2075 * t7983 - t2114 * t7890 - t2165 * t7787 - F::cast_from(2.0_f64) * t7266 * t7796 - t32674 - t33345 - t33360 - t33361 + t33364 + t33365 - t33367;
    t34115
}
