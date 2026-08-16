//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1823/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1823(t6041: f64, t72: f64, t686: f64, t874: f64, t10661: f64, t10923: f64, t10925: f64, t10939: f64, t10948: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t14546: f64, t14951: f64, t14972: f64, t1559: f64, t18525: f64, t18677: f64, t18681: f64, t18699: f64, t4366: f64, t4504: f64, t6022: f64, t820: f64) -> (f64, f64, f64) {
    let t18761 = t6041 * t72;
    let t18763 = t874 * t18761 * t686;
    let t18782 = -0.26019841438354088051e-1_f64 * t14951 - 0.73171657588172351096e-2_f64 * t10923 + 0.65049603595885220126e-3_f64 * t10925 + 0.26341796731742046394e1_f64 * t4504 * t18681 * t4366 + 0.9757440539382783019e-2_f64 * t18763 + 0.13170898365871023197e1_f64 * t820 * t10661 * t6022 + t10939 + 0.13170898365871023197e1_f64 * t4504 * t18699 * t4366 - t10948 - 0.13170898365871023197e1_f64 * t820 * t14972 * t1559 - 0.65049603595885220126e-3_f64 * t10964 + 0.73171657588172351096e-2_f64 * t10966 + t10969 - t10971 - 0.39512695097613069591e1_f64 * t14546 * t18677 * t18525 + 0.39512695097613069591e1_f64 * t4504 * t18677 * t4366;
    (t18761, t18763, t18782)
}
