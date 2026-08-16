//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 763/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk763<F: Float>(t3799: F, t5038: F, t13643: F, t13648: F, t18032: F, t18035: F, t18038: F, t18040: F, t18081: F, t21184: F, t21188: F, t21190: F, t21194: F, t21198: F, t21202: F, t21206: F, t21213: F, t21216: F, t21218: F, t9637: F) -> (F, F) {
    let t21220 = t3799 * t5038;
    let t21222 = F::cast_from(0.18727458458024691358e0_f64) * t18081 + F::cast_from(0.63843608379629629629e-2_f64) * t18032 + F::cast_from(0.85124811172839506172e-2_f64) * t18035 - F::cast_from(0.12768721675925925926e-1_f64) * t18038 - F::cast_from(0.3404992446913580247e-1_f64) * t18040 + F::cast_from(0.19862455940329218107e-1_f64) * t21184 + F::cast_from(0.6384360837962962963e-2_f64) * t21188 + F::cast_from(0.10214977340740740741e0_f64) * t21190 - F::cast_from(0.51074886703703703704e-1_f64) * t21194 + F::cast_from(0.25537443351851851852e-1_f64) * t21198 + F::cast_from(0.38306165027777777778e-1_f64) * t21202 - F::cast_from(0.38306165027777777778e-1_f64) * t21206 + F::cast_from(0.3404992446913580247e-1_f64) * t13643 + t9637 - F::cast_from(0.87394806137448559671e0_f64) * t21213 - F::cast_from(0.42562405586419753086e-2_f64) * t13648 + F::cast_from(0.18727458458024691358e0_f64) * t21216 - F::cast_from(0.51074886703703703705e-1_f64) * t21218 - F::cast_from(0.68099848938271604939e-1_f64) * t21220;
    (t21220, t21222)
}
