//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3405/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3405(t324: f64, t63847: f64, t63861: f64, t63875: f64, t63889: f64, t300: f64, t11506: f64, t15542: f64, t6205: f64, t981: f64, t15566: f64, t19153: f64, t3329: f64, t5023: f64, t63673: f64, t63676: f64, t63679: f64, t63681: f64, t63683: f64, t63685: f64, t63820: f64, t63826: f64, t63827: f64, t63833: f64, t63835: f64) -> (f64, f64, f64, f64) {
    let t63892 = (t63847 + t63861 + t63875 + t63889) * t324;
    let t63894 = 0.19751673498613801407e-1_f64 * t300 * t63892;
    let t63898 = 0.10254018858216406658e4_f64 * t981 * t11506 * t6205 * t15542;
    let t63899 = 8.0_f64 * t15566 * t5023 * t63827 - t19153 * t3329 * t5023 - t63673 - t63676 - t63679 + t63681 - t63683 + t63685 - t63820 + t63826 - t63833 - t63835 + t63894 - t63898;
    (t63892, t63894, t63898, t63899)
}
