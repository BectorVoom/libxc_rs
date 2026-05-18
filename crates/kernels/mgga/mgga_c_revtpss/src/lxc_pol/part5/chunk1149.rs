//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1149/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1149<F: Float>(t5962: F, t854: F, t236: F, t807: F, t2476: F, t5966: F, t10717: F, t10719: F, t10723: F, t10746: F, t10749: F, t14780: F, t14783: F, t14817: F, t14820: F, t14823: F) -> F {
    let t18348 = t854 * t5962;
    let t18349 = t236 * t18348;
    let t18350 = t807 * t18349;
    let t18352 = t2476 * t5966;
    let t18353 = t236 * t18352;
    let t18354 = t807 * t18353;
    let t18361 = F::new(0.2032800112371413129e-4) * t14780 + t14783 + F::new(0.54208002996571016772e-3) * t10717 - F::new(0.76220476654346199061e-4) * t10719 + F::new(0.28582678745379824648e-4) * t18350 - F::new(0.14291339372689912324e-3) * t18354 - F::new(0.22675591804667994221e-1) * t10723 + F::new(0.25410001404642664112e-5) * t10746 - F::new(0.18071592998981862716e-4) * t10749 - F::new(0.36143185997963725434e-4) * t14817 + F::new(0.50820002809285328224e-5) * t14820 - t14823;
    t18361
}
