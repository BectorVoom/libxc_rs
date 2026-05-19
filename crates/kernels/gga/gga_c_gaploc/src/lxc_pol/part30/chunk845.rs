//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 845/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk845<F: Float>(t4820: F, t7893: F, t1424: F, t1429: F, t1430: F, t1646: F, t2891: F, t4418: F, t4849: F, t6626: F, t6722: F, t6724: F, t6732: F, t6793: F, t6824: F, t6841: F, t6845: F, t6847: F, t6849: F, t6856: F, t6860: F, t8226: F, t8229: F, t8233: F, t8238: F, t8248: F) -> F {
    let t8251 = t4820 * t7893;
    let t8256 = F::cast_from(0.17875244975925213335e0_f64) * t6626 - F::cast_from(0.1022478025437886658e1_f64) * t4849 * t2891 - F::cast_from(0.59644551483876721719e0_f64) * t6722 - F::cast_from(0.59584149919750711116e-1_f64) * t6724 + F::cast_from(0.79445533226334281486e-1_f64) * t1429 * t8226 - F::cast_from(0.79445533226334281486e-1_f64) * t8229 * t1424 + F::cast_from(0.29792074959875355558e-1_f64) * t6732 - F::cast_from(0.92686455430723328401e-1_f64) * t1429 * t8233 - F::cast_from(0.11916829983950142223e0_f64) * t6793 - F::cast_from(0.71500979903700853338e0_f64) * t8238 * t1646 + F::cast_from(0.29792074959875355558e-1_f64) * t6841 + F::cast_from(0.59584149919750711116e-1_f64) * t6845 - F::cast_from(0.19171462976960374838e0_f64) * t6847 + F::cast_from(0.85206502119823888168e-1_f64) * t6849 - F::cast_from(0.2698205900461089792e0_f64) * t6856 + F::cast_from(0.59584149919750711116e-1_f64) * t6860 + F::cast_from(0.23833659967900284446e0_f64) * t8248 * t1430 - F::cast_from(0.15889106645266856297e0_f64) * t6824 * t8251 + F::cast_from(0.51123901271894332905e0_f64) * t4418 * t2891;
    t8256
}
