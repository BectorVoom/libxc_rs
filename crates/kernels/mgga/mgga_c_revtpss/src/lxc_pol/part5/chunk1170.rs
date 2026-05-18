//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1170/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1170<F: Float>(t18444: F, t4364: F, t837: F, t14894: F, t14907: F, t14925: F, t14934: F, t18527: F, t18532: F, t18618: F, t18623: F, t18629: F, t18634: F, t18639: F, t18644: F, t18647: F, t2745: F, t4362: F, t825: F) -> F {
    let t18651 = t4364 * t18444 * t837;
    let t18654 = -F::new(0.12862205435420921092e-2) * t14894 * t18527 - F::new(0.12705000702321332056e-4) * t18532 - F::new(0.21437009059034868486e-3) * t825 * t18618 - F::new(0.12705000702321332056e-4) * t18623 - F::new(0.80031500487063509015e-2) * t14907 - t14925 + F::new(0.50820002809285328224e-4) * t14934 + F::new(0.85748036236139473944e-3) * t2745 * t18629 + F::new(0.85748036236139473944e-3) * t4362 * t18634 + F::new(0.17149607247227894789e-2) * t2745 * t18639 + F::new(0.10164000561857065645e-3) * t18644 + F::new(0.17149607247227894789e-2) * t2745 * t18647 - F::new(0.21437009059034868486e-3) * t2745 * t18651;
    t18654
}
