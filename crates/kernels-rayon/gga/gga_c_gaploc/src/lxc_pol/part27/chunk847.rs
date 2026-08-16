//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 847/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk847(t4820: f64, t7893: f64, t1424: f64, t1429: f64, t1430: f64, t1646: f64, t2891: f64, t4418: f64, t4849: f64, t6626: f64, t6722: f64, t6724: f64, t6732: f64, t6793: f64, t6824: f64, t6841: f64, t6845: f64, t6847: f64, t6849: f64, t6856: f64, t6860: f64, t8226: f64, t8229: f64, t8233: f64, t8238: f64, t8248: f64) -> f64 {
    let t8251 = t4820 * t7893;
    let t8256 = 0.17875244975925213335e0_f64 * t6626 - 0.1022478025437886658e1_f64 * t4849 * t2891 - 0.59644551483876721719e0_f64 * t6722 - 0.59584149919750711116e-1_f64 * t6724 + 0.79445533226334281486e-1_f64 * t1429 * t8226 - 0.79445533226334281486e-1_f64 * t8229 * t1424 + 0.29792074959875355558e-1_f64 * t6732 - 0.92686455430723328401e-1_f64 * t1429 * t8233 - 0.11916829983950142223e0_f64 * t6793 - 0.71500979903700853338e0_f64 * t8238 * t1646 + 0.29792074959875355558e-1_f64 * t6841 + 0.59584149919750711116e-1_f64 * t6845 - 0.19171462976960374838e0_f64 * t6847 + 0.85206502119823888168e-1_f64 * t6849 - 0.2698205900461089792e0_f64 * t6856 + 0.59584149919750711116e-1_f64 * t6860 + 0.23833659967900284446e0_f64 * t8248 * t1430 - 0.15889106645266856297e0_f64 * t6824 * t8251 + 0.51123901271894332905e0_f64 * t4418 * t2891;
    t8256
}
