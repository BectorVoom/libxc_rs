//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1156/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1156<F: Float>(t1443: F, t676: F, t28125: F, t8392: F, t10007: F, t108092: F, t109747: F, t13830: F, t13885: F, t14053: F, t14058: F, t14129: F, t14175: F, t1901: F, t242: F, t24705: F, t24737: F, t24747: F, t2574: F, t265: F, t27986: F, t28140: F, t28284: F, t28308: F, t28340: F, t3837: F, t3876: F, t3898: F, t3977: F, t446: F, t51901: F, t6074: F, t6088: F, t684: F, t729: F, t97733: F, t9787: F) -> (F,) {
    let t110751 = t676 * t1443;
    let t110796 = 2.0 / 27.0 * t8392 * t28125;
    let t110797 = -4.0 / 9.0 * t1901 * t51901 * t28340 - 4.0 / 9.0 * t1901 * t14175 * t27986 * t684 - 4.0 / 3.0 * t1901 * t110751 * t14129 - 4.0 * t1901 * t28140 * t6074 * t14053 - 4.0 / 3.0 * t1901 * t13885 * t24737 * t14058 + 2.0 / 9.0 * t1901 * t97733 * t3898 + 2.0 / 9.0 * t1901 * t97733 * t3876 + 2.0 / 9.0 * t1901 * t9787 * t28308 + 4.0 / 3.0 * t446 * t2574 * t265 * t108092 + 2.0 / 3.0 * t446 * t729 * t13830 * t6088 + 2.0 / 3.0 * t446 * t729 * t3977 * t24705 - 2.0 / 3.0 * t446 * t242 * t109747 - 4.0 * t1901 * t28140 * t24747 * t3837 - 2.0 / 9.0 * t1901 * t10007 * t28284 * t684 - t110796;
    (t110797,)
}
