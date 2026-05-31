//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1082/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1082<F: Float>(t1882: F, t35582: F, t35680: F, t35721: F, t10007: F, t109926: F, t1175: F, t13830: F, t13885: F, t14127: F, t14163: F, t142267: F, t142269: F, t149748: F, t150056: F, t150060: F, t150912: F, t151365: F, t1901: F, t242: F, t24737: F, t2574: F, t265: F, t27983: F, t28128: F, t28255: F, t28340: F, t28349: F, t33307: F, t33460: F, t33620: F, t33728: F, t35724: F, t3880: F, t3977: F, t42334: F, t446: F, t53662: F, t684: F, t729: F, t7502: F, t97777: F) -> F {
    let t152230 = t1882 * t35582;
    let t152247 = t1882 * t35680;
    let t152282 = t1882 * t35721;
    let t152284 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t13830 * t7502 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t3977 * t33728 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152230 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t149748 - t446 * t729 * t265 * t150912 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2574 * t3977 * t33620 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t2574 * t1175 * t33307 + t152247 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t13885 * t24737 * t28255 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t14127 * t28128 * t27983 - t142267 / F::cast_from(27.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t109926 * t28340 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t97777 * t28349 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t10007 * t35724 * t684 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t53662 * t150056 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t42334 * t33460 * t3880 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t14163 * t150060 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t151365 + t142269 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152282;
    t152284
}
