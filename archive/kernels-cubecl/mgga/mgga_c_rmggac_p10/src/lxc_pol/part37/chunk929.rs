//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 929/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk929<F: Float>(t73957: F, t73960: F, t73963: F, t73966: F, t68418: F, t73989: F, t73999: F, t74003: F, t15621: F, t7487: F, t74013: F, t74018: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t76802 = F::cast_from(0.30487649791575028312e-3_f64) * t73957;
    let t76803 = F::cast_from(0.40911992481368012596e-1_f64) * t73960;
    let t76804 = F::cast_from(0.81823984962736025192e-1_f64) * t73963;
    let t76805 = F::cast_from(0.40911992481368012596e-1_f64) * t73966;
    let t76808 = F::cast_from(0.60611291211334054834e-6_f64) * t68418;
    let t76814 = F::cast_from(0.11634323970834742769e-4_f64) * t73989;
    let t76816 = F::cast_from(0.23268647941669485538e-4_f64) * t73999;
    let t76817 = F::cast_from(0.85129199786595678799e-5_f64) * t74003;
    let t76819 = t7487 * t15621;
    let t76820 = F::cast_from(0.96056421943322389208e-3_f64) * t76819;
    let t76821 = F::cast_from(0.1276937996798935182e-4_f64) * t74013;
    let t76823 = F::cast_from(0.1276937996798935182e-4_f64) * t74018;
    (t76802, t76803, t76804, t76805, t76808, t76814, t76816, t76817, t76820, t76821, t76823)
}
