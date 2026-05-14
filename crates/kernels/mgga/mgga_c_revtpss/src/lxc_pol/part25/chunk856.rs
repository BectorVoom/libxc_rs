//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 856/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk856<F: Float>(t10905: F, t2732: F, t10853: F, t10855: F, t10859: F, t10863: F, t10870: F, t10874: F, t10878: F, t10881: F, t10885: F, t10888: F, t10891: F, t10893: F, t10896: F, t10900: F, t10902: F, t2721: F, t799: F, t825: F) -> (F,) {
    let t10906 = t10905 * t2732;
    let t10908 = 0.76230004213927992337e-4 * t10853 + 0.30011812682648815881e-2 * t10855 - 0.60023625365297631762e-2 * t10859 + 0.12862205435420921092e-2 * t2721 * t10863 - 0.12862205435420921092e-2 * t10870 * t10874 - 0.21437009059034868486e-3 * t825 * t10878 + 0.30011812682648815881e-2 * t10881 - t10885 + 0.30492001685571196935e-4 * t10888 - 35.0 / 72.0 * t10891 + 7.0 / 48.0 * t10893 - t799 * t10896 / 48.0 - t10900 * t10902 / 4.0 - 7.0 / 16.0 * t10906;
    (t10908,)
}
