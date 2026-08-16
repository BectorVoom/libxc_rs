//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 851/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk851(t105: f64, t13254: f64, t13299: f64, t13323: f64, t169: f64, t172: f64, t2268: f64, t2343: f64, t380: f64, t44549: f64, t44552: f64, t44553: f64, t44556: f64, t44559: f64, t44560: f64, t44564: f64, t44572: f64, t44574: f64, t44576: f64, t44578: f64, t44579: f64, t44580: f64, t44601: f64, t44609: f64, t452: f64, t492: f64, t6305: f64) -> f64 {
    let t44615 = -t44549 + t44552 + t44553 + t44556 + t44559 + 0.1138200265427045984e0_f64 * t2268 * t2343 * t44560 + 0.1138200265427045984e0_f64 * t2268 * t2343 * t44564 + 0.1138200265427045984e0_f64 * t6305 * t13254 + t44572 - t44574 + t44576 - t44578 + t44579 + t44580 + 0.28455006635676149599e-1_f64 * t105 * t452 * t44601 * t169 * t172 - 0.37940008847568199465e-1_f64 * t380 * t13323 - 0.28455006635676149599e-1_f64 * t105 * t492 * t44609 + 0.37940008847568199465e-1_f64 * t380 * t13299;
    t44615
}
