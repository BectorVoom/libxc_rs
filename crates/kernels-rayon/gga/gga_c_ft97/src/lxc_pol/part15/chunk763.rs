//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 763/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk763(t3799: f64, t5038: f64, t13643: f64, t13648: f64, t18032: f64, t18035: f64, t18038: f64, t18040: f64, t18081: f64, t21184: f64, t21188: f64, t21190: f64, t21194: f64, t21198: f64, t21202: f64, t21206: f64, t21213: f64, t21216: f64, t21218: f64, t9637: f64) -> (f64, f64) {
    let t21220 = t3799 * t5038;
    let t21222 = 0.18727458458024691358e0_f64 * t18081 + 0.63843608379629629629e-2_f64 * t18032 + 0.85124811172839506172e-2_f64 * t18035 - 0.12768721675925925926e-1_f64 * t18038 - 0.3404992446913580247e-1_f64 * t18040 + 0.19862455940329218107e-1_f64 * t21184 + 0.6384360837962962963e-2_f64 * t21188 + 0.10214977340740740741e0_f64 * t21190 - 0.51074886703703703704e-1_f64 * t21194 + 0.25537443351851851852e-1_f64 * t21198 + 0.38306165027777777778e-1_f64 * t21202 - 0.38306165027777777778e-1_f64 * t21206 + 0.3404992446913580247e-1_f64 * t13643 + t9637 - 0.87394806137448559671e0_f64 * t21213 - 0.42562405586419753086e-2_f64 * t13648 + 0.18727458458024691358e0_f64 * t21216 - 0.51074886703703703705e-1_f64 * t21218 - 0.68099848938271604939e-1_f64 * t21220;
    (t21220, t21222)
}
