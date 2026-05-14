//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 861/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk861<F: Float>(t10447: F, t4261: F, t309: F, t799: F, t1248: F, t2842: F, t2867: F, t4152: F, t8392: F, t1882: F, t4173: F, t1212: F, t2844: F, t2843: F, t840: F, t2682: F) -> (F, F, F, F, F, F) {
    let t15455 = t10447 * t4261;
    let t15460 = t799 * t309;
    let t15461 = t2842 * t1248;
    let t15462 = t15461 * t2867;
    let t15463 = t15460 * t15462;
    let t15467 = 2.0 / 27.0 * t8392 * t4152;
    let t15471 = 2.0 / 27.0 * t1882 * t4173;
    let t15472 = t1212 * t2844;
    let t15474 = t840 * t2843 * t15472;
    let t15477 = t1212 * t2682;
    (t15455, t15463, t15467, t15471, t15474, t15477)
}
