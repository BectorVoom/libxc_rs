//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1974/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1974<F: Float>(t1361: F, t1799: F, t22690: F, t22792: F, t5227: F, t6916: F, t1998: F, t236: F, t5187: F, t6926: F, t22784: F, t22795: F, t26255: F, t26258: F, t26260: F, t26262: F, t26266: F, t26268: F) -> (F, F, F) {
    let t26271 = t22690 * t1361 * t1799;
    let t26272 = t22792 * t26271;
    let t26274 = t6916 * t5227;
    let t26277 = t1998 * t236 * t5187;
    let t26278 = t6926 * t26277;
    let t26280 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t26255 - t26258 / F::cast_from(384.0_f64) - t26260 / F::cast_from(384.0_f64) - t26262 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t22784 + F::cast_from(0.20186378047070195427e-3_f64) * t22795 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t26266 + F::cast_from(0.84782787797694820792e-2_f64) * t26268 + F::cast_from(0.20186378047070195427e-3_f64) * t26272 - t26274 / F::cast_from(48.0_f64) - F::cast_from(0.12111826828242117256e-2_f64) * t26278;
    (t26271, t26277, t26280)
}
