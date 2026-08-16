//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1322/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1322<F: Float>(t35069: F, t35071: F, t35074: F, t35077: F, t35080: F, t35083: F, t35086: F, t35090: F, t35093: F, t35095: F, t35097: F, t35135: F, t35137: F, t35141: F, t35143: F, t35146: F, t35149: F, t35152: F, t35155: F, t35157: F, t35160: F, t35162: F) -> (F, F) {
    let t38396 = F::cast_from(0.53038229502229385592e-6_f64) * t35069 - F::cast_from(0.84412963981222021456e-7_f64) * t35071 - F::cast_from(0.5397743055555555556e-4_f64) * t35074 + F::cast_from(0.11594181388521408695e-4_f64) * t35077 - F::cast_from(0.8096614583333333334e-3_f64) * t35080 - F::cast_from(0.36620703859188537988e-5_f64) * t35083 + F::cast_from(0.66920900371692798767e-7_f64) * t35086 + F::cast_from(0.20596571349374880758e-5_f64) * t35090 - F::cast_from(0.31433990684987949196e-7_f64) * t35093 + F::cast_from(0.54024296947982093732e-5_f64) * t35095 + F::cast_from(0.2318836277704281739e-4_f64) * t35097;
    let t38422 = -F::cast_from(0.86569887700959851589e-3_f64) * t35135 - F::cast_from(0.6487109086417285278e-2_f64) * t35137 - F::cast_from(0.12310223913928211462e-7_f64) * t35141 - F::cast_from(0.54024296947982093732e-5_f64) * t35143 + F::cast_from(0.22745373045674261828e-5_f64) * t35146 - F::cast_from(0.12891236337347420109e-7_f64) * t35149 + F::cast_from(0.67460644627686456801e-8_f64) * t35152 + F::cast_from(0.11066378711890822966e-7_f64) * t35155 + F::cast_from(0.19808908880926767702e-4_f64) * t35157 + F::cast_from(0.67632724766374884054e-5_f64) * t35160 - F::cast_from(0.16038463156432184077e-5_f64) * t35162;
    (t38396, t38422)
}
