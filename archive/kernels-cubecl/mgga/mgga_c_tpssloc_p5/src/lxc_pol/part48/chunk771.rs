//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 771/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk771<F: Float>(t2047: F, t2591: F, t23042: F, t23044: F, t23049: F, t23051: F, t23054: F, t23057: F, t23059: F, t23063: F, t23067: F, t23070: F, t23073: F, t23081: F, t23084: F, t23087: F, t23090: F) -> (F, F) {
    let t24200 = t2591 * t2047;
    let t24217 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t23042 - t23044 / F::cast_from(768.0_f64) + t23049 / F::cast_from(384.0_f64) - t23051 / F::cast_from(768.0_f64) - t23054 / F::cast_from(384.0_f64) + t23057 / F::cast_from(8.0_f64) - t23059 / F::cast_from(24.0_f64) + F::cast_from(0.33913115119077928316e-1_f64) * t23063 - F::cast_from(0.24223653656484234512e-2_f64) * t23067 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t23070 + F::cast_from(0.80745512188280781706e-3_f64) * t23073 + F::cast_from(0.16956557559538964158e-1_f64) * t23081 + F::cast_from(0.56521858531796547194e-2_f64) * t23084 - F::cast_from(0.40372756094140390853e-3_f64) * t23087 - F::cast_from(0.40372756094140390853e-3_f64) * t23090;
    (t24200, t24217)
}
