//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1030/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1030(t113: f64, t119824: f64, t119826: f64, t119830: f64, t120669: f64, t123844: f64, t123947: f64, t123975: f64, t1307: f64, t15868: f64, t1983: f64, t22574: f64, t23938: f64, t24432: f64, t26977: f64, t27180: f64, t27219: f64, t31304: f64, t32186: f64, t32194: f64, t32212: f64, t33790: f64, t5161: f64, t6876: f64, t6879: f64, t7042: f64, t7685: f64, t7806: f64, t7904: f64, t7939: f64, t7941: f64, t7943: f64, t8804: f64) -> f64 {
    let t123981 = -t119824 - t119826 - t119830 - 2.0_f64 * t31304 * t7943 - t113 * (t123844 + t123947) + 2.0_f64 * t31304 * t7941 - 3.0_f64 * t1983 * t32212 * t120669 - t1983 * t8804 * t15868 - 4.0_f64 * t7042 * t27219 - 4.0_f64 * t23938 * t7806 - 4.0_f64 * t26977 * t7806 - 4.0_f64 * t7042 * t27180 - 6.0_f64 * t22574 * t24432 * t7939 * t1307 - 2.0_f64 * t7685 * t32194 - 3.0_f64 * t6876 * t33790 + 6.0_f64 * t31304 * t7904 + 3.0_f64 * t1983 * t123975 * t6879 - t1983 * t32186 * t5161;
    t123981
}
