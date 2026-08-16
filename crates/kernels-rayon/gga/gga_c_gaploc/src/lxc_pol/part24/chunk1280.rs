//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1280/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1280(t33047: f64, t10938: f64, t1980: f64, t2028: f64, t32757: f64, t32970: f64, t10677: f64, t10948: f64, t10999: f64, t1391: f64, t1392: f64, t1445: f64, t1858: f64, t1865: f64, t2087: f64, t28289: f64, t28291: f64, t28297: f64, t28308: f64, t28312: f64, t32234: f64, t33030: f64, t33033: f64, t33034: f64, t33041: f64, t5748: f64, t7769: f64, t787: f64, t825: f64) -> f64 {
    let t33048 = 0.29792074959875355558e-1_f64 * t33047;
    let t33055 = 0.79445533226334281486e-1_f64 * t1980 * t10938 * t2028;
    let t33060 = 0.50050685932590597338e1_f64 * t32757 * t32970;
    let t33061 = -t28289 - t28291 + t28297 - 0.25025342966295298669e1_f64 * t10948 * t7769 - t28308 + t28312 + t33030 + t33033 + 0.27606906686822939767e2_f64 * t5748 * t1445 * t33034 * t1865 + t33041 - 0.11360866949309851756e0_f64 * t825 * t1391 * t1392 * t10677 - t33048 - 0.79445533226334281486e-1_f64 * t787 * t1858 * t10999 * t2028 - t33055 - 0.62115540045351614476e2_f64 * t2087 * t1445 * t32234 - t33060;
    t33061
}
