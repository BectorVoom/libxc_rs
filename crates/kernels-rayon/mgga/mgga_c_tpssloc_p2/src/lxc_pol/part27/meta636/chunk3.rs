//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2147/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2147(t13306: f64, t23146: f64, t13231: f64, t25084: f64, t81789: f64, t81795: f64, t81797: f64, t81799: f64, t81810: f64, t81825: f64, t81836: f64, t81850: f64, t81853: f64, t87263: f64, t87268: f64, t87271: f64, t87273: f64, t87274: f64, t87276: f64, t87278: f64) -> f64 {
    let t87280 = t23146 * t13306;
    let t87284 = t25084 * t13231;
    let t87286 = t87263 - 0.63250651214153279005e-2_f64 * t81789 - 0.14130464632949136799e-2_f64 * t81795 - 0.28260929265898273598e-2_f64 * t81797 + 7.0_f64 / 144.0_f64 * t81799 - t87268 + 7.0_f64 / 2304.0_f64 * t81810 - t87271 + t87273 + t87274 / 768.0_f64 + t87276 / 384.0_f64 + t87278 / 384.0_f64 + t87280 / 384.0_f64 + 7.0_f64 / 1152.0_f64 * t81825 - 0.16956557559538964159e-1_f64 * t81836 - t81850 - t81853 - t87284 / 96.0_f64;
    t87286
}
