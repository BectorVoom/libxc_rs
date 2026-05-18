//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1277/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1277<F: Float>(t33047: F, t10938: F, t1980: F, t2028: F, t32757: F, t32970: F, t10677: F, t10948: F, t10999: F, t1391: F, t1392: F, t1445: F, t1858: F, t1865: F, t2087: F, t28289: F, t28291: F, t28297: F, t28308: F, t28312: F, t32234: F, t33030: F, t33033: F, t33034: F, t33041: F, t5748: F, t7769: F, t787: F, t825: F) -> F {
    let t33048 = F::new(0.29792074959875355558e-1) * t33047;
    let t33055 = F::new(0.79445533226334281486e-1) * t1980 * t10938 * t2028;
    let t33060 = F::new(0.50050685932590597338e1) * t32757 * t32970;
    let t33061 = -t28289 - t28291 + t28297 - F::new(0.25025342966295298669e1) * t10948 * t7769 - t28308 + t28312 + t33030 + t33033 + F::new(0.27606906686822939767e2) * t5748 * t1445 * t33034 * t1865 + t33041 - F::new(0.11360866949309851756e0) * t825 * t1391 * t1392 * t10677 - t33048 - F::new(0.79445533226334281486e-1) * t787 * t1858 * t10999 * t2028 - t33055 - F::new(0.62115540045351614476e2) * t2087 * t1445 * t32234 - t33060;
    t33061
}
