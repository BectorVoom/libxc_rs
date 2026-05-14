//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1257/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1257<F: Float>(t23383: F, t446: F, t6290: F, t908: F, t1881: F, t5414: F, t13000: F, t13096: F, t18385: F, t20856: F, t20859: F, t20861: F, t20863: F, t20866: F, t20870: F, t23381: F, t9267: F, t9270: F, t9278: F, t9281: F) -> (F,) {
    let t23384 = t446 * t23383;
    let t23386 = t6290 * t908;
    let t23387 = t1881 * t5414;
    let t23389 = -t20856 / 8.0 - t20859 / 16.0 + t13096 + t20861 / 8.0 + t20863 / 16.0 - t20866 / 16.0 + 2.0 * t18385 - t20870 / 16.0 - t9278 + t9267 - t23381 / 16.0 + t9281 - t23384 / 8.0 + t23386 + t23387 / 8.0 - t9270 + t13000;
    (t23389,)
}
