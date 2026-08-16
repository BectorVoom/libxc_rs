//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1586/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1586(t10905: f64, t2732: f64, t10853: f64, t10855: f64, t10859: f64, t10863: f64, t10870: f64, t10874: f64, t10878: f64, t10881: f64, t10885: f64, t10888: f64, t10891: f64, t10893: f64, t10896: f64, t10900: f64, t10902: f64, t2721: f64, t799: f64, t825: f64) -> (f64, f64) {
    let t10906 = t10905 * t2732;
    let t10908 = 0.76230004213927992337e-4_f64 * t10853 + 0.30011812682648815881e-2_f64 * t10855 - 0.60023625365297631762e-2_f64 * t10859 + 0.12862205435420921092e-2_f64 * t2721 * t10863 - 0.12862205435420921092e-2_f64 * t10870 * t10874 - 0.21437009059034868486e-3_f64 * t825 * t10878 + 0.30011812682648815881e-2_f64 * t10881 - t10885 + 0.30492001685571196935e-4_f64 * t10888 - 35.0_f64 / 72.0_f64 * t10891 + 7.0_f64 / 48.0_f64 * t10893 - t799 * t10896 / 48.0_f64 - t10900 * t10902 / 4.0_f64 - 7.0_f64 / 16.0_f64 * t10906;
    (t10906, t10908)
}
