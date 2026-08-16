//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1193/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1193(t5962: f64, t854: f64, t236: f64, t807: f64, t2476: f64, t5966: f64, t10717: f64, t10719: f64, t10723: f64, t10746: f64, t10749: f64, t14780: f64, t14783: f64, t14817: f64, t14820: f64, t14823: f64) -> f64 {
    let t18348 = t854 * t5962;
    let t18349 = t236 * t18348;
    let t18350 = t807 * t18349;
    let t18352 = t2476 * t5966;
    let t18353 = t236 * t18352;
    let t18354 = t807 * t18353;
    let t18361 = 0.2032800112371413129e-4_f64 * t14780 + t14783 + 0.54208002996571016772e-3_f64 * t10717 - 0.76220476654346199061e-4_f64 * t10719 + 0.28582678745379824648e-4_f64 * t18350 - 0.14291339372689912324e-3_f64 * t18354 - 0.22675591804667994221e-1_f64 * t10723 + 0.25410001404642664112e-5_f64 * t10746 - 0.18071592998981862716e-4_f64 * t10749 - 0.36143185997963725434e-4_f64 * t14817 + 0.50820002809285328224e-5_f64 * t14820 - t14823;
    t18361
}
