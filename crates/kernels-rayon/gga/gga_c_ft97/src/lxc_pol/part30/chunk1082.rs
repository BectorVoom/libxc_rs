//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1082/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1082(t1882: f64, t35582: f64, t35680: f64, t35721: f64, t10007: f64, t109926: f64, t1175: f64, t13830: f64, t13885: f64, t14127: f64, t14163: f64, t142267: f64, t142269: f64, t149748: f64, t150056: f64, t150060: f64, t150912: f64, t151365: f64, t1901: f64, t242: f64, t24737: f64, t2574: f64, t265: f64, t27983: f64, t28128: f64, t28255: f64, t28340: f64, t28349: f64, t33307: f64, t33460: f64, t33620: f64, t33728: f64, t35724: f64, t3880: f64, t3977: f64, t42334: f64, t446: f64, t53662: f64, t684: f64, t729: f64, t7502: f64, t97777: f64) -> f64 {
    let t152230 = t1882 * t35582;
    let t152247 = t1882 * t35680;
    let t152282 = t1882 * t35721;
    let t152284 = 2.0_f64 / 3.0_f64 * t446 * t729 * t13830 * t7502 + 2.0_f64 / 3.0_f64 * t446 * t729 * t3977 * t33728 - 2.0_f64 / 9.0_f64 * t152230 + 2.0_f64 / 3.0_f64 * t446 * t242 * t149748 - t446 * t729 * t265 * t150912 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t2574 * t3977 * t33620 + 4.0_f64 / 3.0_f64 * t446 * t2574 * t1175 * t33307 + t152247 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t13885 * t24737 * t28255 - 4.0_f64 / 3.0_f64 * t1901 * t14127 * t28128 * t27983 - t142267 / 27.0_f64 - 4.0_f64 / 9.0_f64 * t1901 * t109926 * t28340 - 2.0_f64 / 9.0_f64 * t1901 * t97777 * t28349 - 2.0_f64 / 9.0_f64 * t1901 * t10007 * t35724 * t684 + 2.0_f64 / 3.0_f64 * t1901 * t53662 * t150056 + 2.0_f64 / 9.0_f64 * t1901 * t42334 * t33460 * t3880 - 4.0_f64 / 9.0_f64 * t1901 * t14163 * t150060 + 4.0_f64 / 3.0_f64 * t446 * t242 * t151365 + t142269 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t152282;
    t152284
}
