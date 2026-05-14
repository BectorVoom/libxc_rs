//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 718/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk718<F: Float>(t13146: F, t5676: F, t23000: F, t33308: F, t9889: F, t11068: F, t2617: F, t7803: F, t13064: F, t2684: F, t7354: F, t10867: F, t1423: F, t3247: F, t13077: F, t28439: F) -> (F, F, F, F, F, F) {
    let t43817 = t5676 * t13146;
    let t43832 = t23000 * t33308 * t9889;
    let t43881 = t7803 * t11068 * t2617;
    let t43904 = t2684 * t7354 * t13064;
    let t43907 = t10867 * t1423 * t3247;
    let t43912 = t13077 * t28439;
    (t43817, t43832, t43881, t43904, t43907, t43912)
}
