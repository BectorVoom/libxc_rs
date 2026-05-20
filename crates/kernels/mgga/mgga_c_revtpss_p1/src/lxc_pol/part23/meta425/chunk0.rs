//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1814/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1814<F: Float>(t18637: F, t2747: F, t4365: F, t10779: F, t14671: F, t6035: F, t10777: F, t14676: F, t18444: F, t4364: F, t837: F, t14894: F, t14907: F, t14925: F, t14934: F, t18527: F, t18532: F, t18618: F, t18623: F, t18629: F, t18634: F, t2745: F, t4362: F, t825: F) -> (F, F, F, F, F, F) {
    let t18639 = t2747 * t4365 * t18637;
    let t18643 = t10779 * t14671 * t6035;
    let t18644 = t10777 * t18643;
    let t18647 = t2747 * t14676 * t6035;
    let t18651 = t4364 * t18444 * t837;
    let t18654 = -F::cast_from(0.12862205435420921092e-2_f64) * t14894 * t18527 - F::cast_from(0.12705000702321332056e-4_f64) * t18532 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t18618 - F::cast_from(0.12705000702321332056e-4_f64) * t18623 - F::cast_from(0.80031500487063509015e-2_f64) * t14907 - t14925 + F::cast_from(0.50820002809285328224e-4_f64) * t14934 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t18629 + F::cast_from(0.85748036236139473944e-3_f64) * t4362 * t18634 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t18639 + F::cast_from(0.10164000561857065645e-3_f64) * t18644 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t18647 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t18651;
    (t18639, t18643, t18644, t18647, t18651, t18654)
}
