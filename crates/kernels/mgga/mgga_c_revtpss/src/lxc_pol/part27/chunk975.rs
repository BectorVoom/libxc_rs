//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 975/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk975<F: Float>(t1026: F, t371: F, t676: F, t1025: F, t271: F, t2857: F, t11144: F, t10356: F, t1012: F, t11150: F, t3252: F, t11156: F, t4919: F) -> (F, F, F, F) {
    let t11817 = t371 * t676 * t1026;
    let t11818 = t1025 * t11817;
    let t11821 = F::new(1.0) / t271 / t2857;
    let t11822 = t11821 * t11144;
    let t11823 = t11822 * t10356;
    let t11824 = t1012 * t11823;
    let t11827 = t3252 * t11150;
    let t11828 = t11827 * t10356;
    let t11829 = t1012 * t11828;
    let t11836 = t4919 * t11156;
    (t11818, t11824, t11829, t11836)
}
