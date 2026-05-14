//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 795/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk795<F: Float>(t549: F, t7981: F, t1397: F, t2897: F, t1402: F, t2783: F, t1359: F, t986: F, t1415: F, t107: F, t7887: F, t544: F, t4820: F, t7893: F, t1424: F, t1429: F, t1430: F, t1646: F, t2891: F, t4418: F, t4849: F, t6626: F, t6722: F, t6724: F, t6732: F, t6793: F, t6824: F, t6841: F, t6845: F, t6847: F, t6849: F, t6856: F, t6860: F) -> (F, F, F, F, F) {
    let t8226 = t549 * t7981;
    let t8229 = t1397 * t2897;
    let t8233 = t1402 * t2783;
    let t8237 = t1359 * t986;
    let t8238 = t1415 * t8237;
    let t8247 = t7887 * t107;
    let t8248 = t544 * t8247;
    let t8251 = t4820 * t7893;
    let t8256 = 0.17875244975925213335e0 * t6626 - 0.1022478025437886658e1 * t4849 * t2891 - 0.59644551483876721719e0 * t6722 - 0.59584149919750711116e-1 * t6724 + 0.79445533226334281486e-1 * t1429 * t8226 - 0.79445533226334281486e-1 * t8229 * t1424 + 0.29792074959875355558e-1 * t6732 - 0.92686455430723328401e-1 * t1429 * t8233 - 0.11916829983950142223e0 * t6793 - 0.71500979903700853338e0 * t8238 * t1646 + 0.29792074959875355558e-1 * t6841 + 0.59584149919750711116e-1 * t6845 - 0.19171462976960374838e0 * t6847 + 0.85206502119823888168e-1 * t6849 - 0.2698205900461089792e0 * t6856 + 0.59584149919750711116e-1 * t6860 + 0.23833659967900284446e0 * t8248 * t1430 - 0.15889106645266856297e0 * t6824 * t8251 + 0.51123901271894332905e0 * t4418 * t2891;
    (t8229, t8237, t8247, t8248, t8256)
}
