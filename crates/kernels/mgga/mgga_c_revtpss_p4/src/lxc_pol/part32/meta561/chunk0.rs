//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1880/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1880<F: Float>(t27349: F, t689: F, t25260: F, t4368: F, t820: F, t844: F, t4462: F, t92951: F, t92963: F, t92966: F, t92969: F, t27253: F, t9775: F) -> (F, F, F, F, F, F, F) {
    let t98892 = t27349 * t689;
    let t98937 = t820 * t25260 * t844 * t4368;
    let t98949 = t92951 * t4462;
    let t98960 = F::cast_from(0.10164000561857065645e-4_f64) * t92963;
    let t98961 = F::cast_from(0.72286371995927450868e-4_f64) * t92966;
    let t98962 = F::new(35.0) / F::new(108.0) * t92969;
    let t98964 = t9775 * t27253;
    (t98892, t98937, t98949, t98960, t98961, t98962, t98964)
}
