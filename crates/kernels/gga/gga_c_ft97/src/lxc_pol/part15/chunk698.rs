//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 698/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk698<F: Float>(t21204: F, t2320: F, t701: F, t207: F, t216: F, t14: F, t228: F, t231: F, t1124: F, t18043: F, t3799: F, t5046: F, t5038: F, t13643: F, t13648: F, t18032: F, t18035: F, t18038: F, t18040: F, t18081: F, t21184: F, t21188: F, t21190: F, t21194: F, t21198: F, t21202: F, t9637: F) -> (F, F, F, F, F, F, F, F) {
    let t21205 = t2320 * t21204;
    let t21206 = t701 * t21205;
    let t21210 = 1.0 / t207 / t216;
    let t21213 = t228 * t21210 * t14 * t231;
    let t21216 = t18043 * t1124;
    let t21218 = t3799 * t5046;
    let t21220 = t3799 * t5038;
    let t21222 = 0.18727458458024691358e0 * t18081 + 0.63843608379629629629e-2 * t18032 + 0.85124811172839506172e-2 * t18035 - 0.12768721675925925926e-1 * t18038 - 0.3404992446913580247e-1 * t18040 + 0.19862455940329218107e-1 * t21184 + 0.6384360837962962963e-2 * t21188 + 0.10214977340740740741e0 * t21190 - 0.51074886703703703704e-1 * t21194 + 0.25537443351851851852e-1 * t21198 + 0.38306165027777777778e-1 * t21202 - 0.38306165027777777778e-1 * t21206 + 0.3404992446913580247e-1 * t13643 + t9637 - 0.87394806137448559671e0 * t21213 - 0.42562405586419753086e-2 * t13648 + 0.18727458458024691358e0 * t21216 - 0.51074886703703703705e-1 * t21218 - 0.68099848938271604939e-1 * t21220;
    (t21205, t21206, t21210, t21213, t21216, t21218, t21220, t21222)
}
