//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 740/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk740<F: Float>(t13287: F, t64: F, t11210: F, t871: F, t39624: F, t39626: F, t39632: F, t39646: F, t39648: F, t39650: F, t42114: F, t42115: F, t44590: F, t493: F, t105: F, t13254: F, t13299: F, t13323: F, t169: F, t172: F, t2268: F, t2343: F, t380: F, t44549: F, t44552: F, t44553: F, t44556: F, t44559: F, t44560: F, t44564: F, t44572: F, t44574: F, t44576: F, t44578: F, t44579: F, t44580: F, t452: F, t492: F, t6305: F) -> (F, F, F) {
    let t44592 = 4.0 / 3.0 * t13287 * t64;
    let t44593 = t11210 * t871;
    let t44595 = 7.0 / 256.0 * t39624;
    let t44596 = 63.0 / 8192.0 * t39626;
    let t44597 = 63.0 / 524288.0 * t39632;
    let t44598 = 21.0 / 524288.0 * t39646;
    let t44599 = 21.0 / 8192.0 * t39648;
    let t44600 = 7.0 / 768.0 * t39650;
    let t44601 = t44590 - t44592 + t44593 / 2.0 + t42114 - t42115 + t44595 + t44596 - t44597 + t44598 - t44599 - t44600;
    let t44609 = t493 * t44601;
    let t44615 = -t44549 + t44552 + t44553 + t44556 + t44559 + 0.1138200265427045984e0 * t2268 * t2343 * t44560 + 0.1138200265427045984e0 * t2268 * t2343 * t44564 + 0.1138200265427045984e0 * t6305 * t13254 + t44572 - t44574 + t44576 - t44578 + t44579 + t44580 + 0.28455006635676149599e-1 * t105 * t452 * t44601 * t169 * t172 - 0.37940008847568199465e-1 * t380 * t13323 - 0.28455006635676149599e-1 * t105 * t492 * t44609 + 0.37940008847568199465e-1 * t380 * t13299;
    (t44601, t44609, t44615)
}
