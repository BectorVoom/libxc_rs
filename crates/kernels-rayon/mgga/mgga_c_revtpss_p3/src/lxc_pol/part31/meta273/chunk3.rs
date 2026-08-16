//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1228/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1228(t7160: f64, t7828: f64, t1089: f64, t1668: f64, t7168: f64, t1984: f64, t359: f64, t7810: f64, t1647: f64, t1652: f64, t1696: f64, t1978: f64, t1983: f64, t1986: f64, t342: f64, t7102: f64, t7140: f64, t7144: f64, t7151: f64, t7159: f64, t7167: f64, t7812: f64, t7818: f64, t7822: f64, t7825: f64) -> (f64, f64, f64, f64) {
    let t7829 = t7160 * t7828;
    let t7833 = t7168 * t1668 * t1089;
    let t7837 = t1984 * t359 * t7810;
    let t7840 = 0.65854491829355115987e0_f64 * t1647 * t1978 - 0.65854491829355115987e0_f64 * t7102 * t1652 + 0.65854491829355115987e0_f64 * t342 * t7812 - 0.65854491829355115987e0_f64 * t7140 * t1696 - 0.8673628188205199462e0_f64 * t7144 * t7818 + 0.8673628188205199462e0_f64 * t7151 * t7822 - 0.4336814094102599731e0_f64 * t7825 * t1986 + 0.8673628188205199462e0_f64 * t7159 * t7829 - 0.4336814094102599731e0_f64 * t7167 * t7833 - 0.4336814094102599731e0_f64 * t1983 * t7837;
    (t7829, t7833, t7837, t7840)
}
