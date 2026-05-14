//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1236/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1236<F: Float>(t3758: F, t6777: F, t5001: F, t30712: F, t6051: F, t109108: F, t1100: F, t6042: F, t79851: F, t30708: F, t6050: F, t1417: F, t109014: F, t30621: F, t27658: F, t109080: F, t109316: F, t1095: F, t1407: F, t17819: F, t17950: F, t17960: F, t18084: F, t231: F, t24346: F, t27494: F, t27506: F, t27584: F, t27588: F, t27596: F, t3766: F, t3773: F, t3774: F, t52593: F, t6043: F, t6045: F, t6047: F, t680: F, t79821: F, t96660: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t123754 = t3758 * t6777;
    let t123759 = sigma2 * t5001;
    let t123766 = t30712 * t6051;
    let t123768 = t1100 * t109108;
    let t123781 = t79851 * t6042;
    let t123784 = t30708 * t6050;
    let t123785 = t1417 * t123784;
    let t123787 = t109014 * t30621;
    let t123788 = t27658 * t123787;
    let t123808 = -0.93019603785751168e-1 * t96660 * t680 * t1095 * t17960 - 0.93019603785751168e-2 * t24346 * t109080 * t17950 + 0.76612330055555555556e-1 * t123781 * t6047 - 0.62424861526748971193e-1 * t123785 - 0.10091343167942740398e-3 * t123788 + 0.38306165027777777778e-1 * t6043 * t6045 * t231 * t18084 - 0.20429954681481481482e0 * t6043 * t27506 * t27588 - 0.27568129967481981593e-3 * t3774 * t27584 * t27596 + 0.64109413167231678974e-5 * t17819 * t3773 * t27494 * t1095 - 0.17782141943527538963e-1 * t3766 * t52593 * t1407 * t79821 - t109316;
    (t123754, t123759, t123766, t123768, t123784, t123787, t123808)
}
