//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1319/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1319<F: Float>(t35069: F, t35071: F, t35074: F, t35077: F, t35080: F, t35083: F, t35086: F, t35090: F, t35093: F, t35095: F, t35097: F, t35135: F, t35137: F, t35141: F, t35143: F, t35146: F, t35149: F, t35152: F, t35155: F, t35157: F, t35160: F, t35162: F) -> (F, F) {
    let t38396 = F::new(0.53038229502229385592e-6) * t35069 - F::new(0.84412963981222021456e-7) * t35071 - F::new(0.5397743055555555556e-4) * t35074 + F::new(0.11594181388521408695e-4) * t35077 - F::new(0.8096614583333333334e-3) * t35080 - F::new(0.36620703859188537988e-5) * t35083 + F::new(0.66920900371692798767e-7) * t35086 + F::new(0.20596571349374880758e-5) * t35090 - F::new(0.31433990684987949196e-7) * t35093 + F::new(0.54024296947982093732e-5) * t35095 + F::new(0.2318836277704281739e-4) * t35097;
    let t38422 = -F::new(0.86569887700959851589e-3) * t35135 - F::new(0.6487109086417285278e-2) * t35137 - F::new(0.12310223913928211462e-7) * t35141 - F::new(0.54024296947982093732e-5) * t35143 + F::new(0.22745373045674261828e-5) * t35146 - F::new(0.12891236337347420109e-7) * t35149 + F::new(0.67460644627686456801e-8) * t35152 + F::new(0.11066378711890822966e-7) * t35155 + F::new(0.19808908880926767702e-4) * t35157 + F::new(0.67632724766374884054e-5) * t35160 - F::new(0.16038463156432184077e-5) * t35162;
    (t38396, t38422)
}
