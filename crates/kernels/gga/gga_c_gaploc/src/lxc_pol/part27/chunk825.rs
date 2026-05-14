//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 825/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk825<F: Float>(t2005: F, t2009: F, t2034: F, t3025: F, t7513: F, t7653: F, t7720: F, t7743: F, t7780: F, t7782: F, t7786: F, t7790: F, t7792: F, t7795: F, t7798: F, t7800: F, t7807: F, t7812: F, t813: F, t8766: F, t8770: F, t8775: F, t8789: F, t8793: F) -> (F,) {
    let t8796 = 0.59584149919750711116e-1 * t7720 - 0.11916829983950142223e0 * t7743 - 0.14300195980740170668e1 * t3025 * t7653 - 0.12269736305254639896e2 * t813 * t8766 - 0.15889106645266856297e0 * t7513 * t8770 + 0.23833659967900284446e0 * t8775 * t2034 + 0.59584149919750711116e-1 * t7780 + 0.29792074959875355558e-1 * t7782 - 0.89376224879626066674e-1 * t7786 - 0.29792074959875355558e-1 * t7790 - 0.59584149919750711116e-1 * t7792 + 0.11916829983950142223e0 * t7795 - 0.2698205900461089792e0 * t7798 + 0.51123901271894332902e0 * t7800 + 0.76685851907841499352e0 * t7807 - 0.76685851907841499352e0 * t7812 - 0.71500979903700853338e0 * t8789 * t2009 + 0.21450293971110256002e1 * t8793 * t2005;
    (t8796,)
}
