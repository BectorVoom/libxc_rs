//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 672/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk672<F: Float>(t1078: F, t7165: F, t1982: F, t1035: F, t1976: F, t1043: F, t1089: F, t1984: F, t359: F, t7135: F, t1000: F, t1097: F, t1978: F, t1983: F, t1986: F, t342: F, t7102: F, t7137: F, t7140: F, t7144: F, t7147: F, t7151: F, t7153: F, t7156: F, t7159: F, t7162: F, t989: F) -> (F, F, F, F, F) {
    let t7166 = t7165 * t1078;
    let t7167 = t1982 * t7166;
    let t7168 = t1035 * t1976;
    let t7170 = t7168 * t1043 * t1089;
    let t7174 = t1984 * t359 * t7135;
    let t7177 = F::cast_from(0.65854491829355115987e0_f64) * t989 * t1978 - F::cast_from(0.65854491829355115987e0_f64) * t7102 * t1000 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t7137 - F::cast_from(0.65854491829355115987e0_f64) * t7140 * t1097 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7147 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t7153 - F::cast_from(0.4336814094102599731e0_f64) * t7156 * t1986 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t7162 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t7170 - F::cast_from(0.4336814094102599731e0_f64) * t1983 * t7174;
    (t7167, t7168, t7170, t7174, t7177)
}
