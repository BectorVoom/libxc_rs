//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1130/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1130<F: Float>(t271: F, t2857: F, t11144: F, t10356: F, t1012: F, t11150: F, t3252: F, t11156: F, t4919: F, t11165: F, t4915: F, t1066: F, t11169: F, t247: F) -> (F, F, F, F, F, F, F, F) {
    let t11821 = F::cast_from(1.0_f64) / t271 / t2857;
    let t11822 = t11821 * t11144;
    let t11823 = t11822 * t10356;
    let t11824 = t1012 * t11823;
    let t11827 = t3252 * t11150;
    let t11828 = t11827 * t10356;
    let t11829 = t1012 * t11828;
    let t11836 = t4919 * t11156;
    let t11839 = t4915 * t11165;
    let t11845 = t247 * t1066 * t11169;
    (t11821, t11823, t11824, t11828, t11829, t11836, t11839, t11845)
}
