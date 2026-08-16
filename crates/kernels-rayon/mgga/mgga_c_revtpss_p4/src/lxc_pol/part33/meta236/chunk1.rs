//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1063/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1063(t1079: f64, t6392: f64, t1076: f64, t1647: f64, t1652: f64, t1680: f64, t1696: f64, t3058: f64, t342: f64, t386: f64, t4747: f64, t4752: f64, t4778: f64, t4935: f64, t6235: f64, t6245: f64, t6251: f64, t6259: f64, t6345: f64, t6351: f64, t995: f64) -> (f64, f64) {
    let t6393 = t1079 * t6392;
    let t6396 = 0.65854491829355115987e0_f64 * t6235 * t386 - 0.13170898365871023197e1_f64 * t4747 * t1652 + 0.13170898365871023197e1_f64 * t1647 * t1680 - 0.13170898365871023197e1_f64 * t4752 * t1696 + 0.13170898365871023197e1_f64 * t3058 * t6245 - 0.13170898365871023197e1_f64 * t4778 * t1652 + 0.13170898365871023197e1_f64 * t995 * t6251 - 0.65854491829355115987e0_f64 * t995 * t6259 + 0.65854491829355115987e0_f64 * t342 * t6345 - 0.13170898365871023197e1_f64 * t4935 * t1696 + 0.13170898365871023197e1_f64 * t1076 * t6351 - 0.65854491829355115987e0_f64 * t1076 * t6393;
    (t6393, t6396)
}
