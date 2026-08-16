//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2591/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2591<F: Float>(t43350: F, t44535: F, t45607: F, t460: F, t487: F, t13045: F, t13147: F, t1770: F, t1209: F, t1284: F, t5412: F, t17306: F, t3754: F) -> (F, F, F, F, F, F) {
    let t59865 = t43350 * t44535;
    let t59871 = t460 * t45607 * t487;
    let t59872 = t43350 * t13045;
    let t59948 = t1770 * t13147;
    let t60008 = t1209 * t1284 * t5412;
    let t60019 = t17306 * t3754;
    (t59865, t59871, t59872, t59948, t60008, t60019)
}
