//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1016/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1016(t10978: f64, t2103: f64, t10717: f64, t1457: f64, t10721: f64, t3470: f64, t8478: f64, t8638: f64, t10948: f64, t10953: f64, t10955: f64, t10958: f64, t10963: f64, t10966: f64, t10967: f64, t10971: f64, t10975: f64, t10977: f64, t2004: f64, t2639: f64, t813: f64, t833: f64) -> (f64, f64, f64) {
    let t10980 = 0.71500979903700853338e0_f64 * t2103 * t10978;
    let t10981 = t1457 * t10717;
    let t10983 = 0.71500979903700853338e0_f64 * t2103 * t10981;
    let t10984 = t1457 * t10721;
    let t10988 = 0.10725146985555128001e1_f64 * t8478 * t3470;
    let t10990 = 0.10725146985555128001e1_f64 * t8638 * t3470;
    let t10991 = -0.10725146985555128001e1_f64 * t10948 * t2639 - t10953 - 0.46011511144704899612e1_f64 * t813 * t10955 + 0.11502877786176224903e2_f64 * t833 * t10958 + t10963 - t10966 + 0.71500979903700853338e0_f64 * t2103 * t10967 - t10971 - t10975 + t10977 + t10980 + t10983 + 0.35750489951850426669e0_f64 * t2004 * t10984 - t10988 - t10990;
    (t10981, t10984, t10991)
}
