//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1193/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1193<F: Float>(t109015: F, t28558: F, t1471: F, t4061: F, t2035: F, t6789: F, t6793: F, t811: F, t820: F, t25057: F, t2735: F, t28637: F, t218: F, t4125: F, t28628: F, t112208: F, t14721: F, t14729: F, t14766: F, t25120: F, t28639: F, t54924: F, t6829: F, t70550: F, t98561: F) -> (F, F, F, F, F, F) {
    let t112219 = 0.26853068634149852184e-1 * t28558 * t109015;
    let t112220 = t4061 * t1471;
    let t112223 = t2035 * t6789;
    let t112226 = t112223 * t6793 * t811 * t820;
    let t112230 = t25057 * t28637 * t2735;
    let t112235 = t218 * t4125;
    let t112237 = t25057 * t112235 * t820;
    let t112244 = t25057 * t28628 * t2735;
    let t112248 = t25057 * t112235 * t811;
    let t112251 = 0.26671200437037037038e0 * t25120 * t6829 - t112219 + 0.90613700826057446696e0 * t112220 * t28639 + 0.21895580739717983994e1 * t70550 * t112226 + 0.45306850413028723348e0 * t14766 * t112230 - 0.45306850413028723348e0 * t14721 * t112230 - 0.90613700826057446696e0 * t14729 * t112237 + 0.13335600218518518519e0 * t98561 + 0.90613700826057446696e0 * t54924 * t112208 - 0.45306850413028723348e0 * t14729 * t112244 + 0.90613700826057446696e0 * t14766 * t112248;
    (t112223, t112226, t112237, t112244, t112248, t112251)
}
