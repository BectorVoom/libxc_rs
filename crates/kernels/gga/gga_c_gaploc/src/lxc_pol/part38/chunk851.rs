//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 851/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk851<F: Float>(t105: F, t13254: F, t13299: F, t13323: F, t169: F, t172: F, t2268: F, t2343: F, t380: F, t44549: F, t44552: F, t44553: F, t44556: F, t44559: F, t44560: F, t44564: F, t44572: F, t44574: F, t44576: F, t44578: F, t44579: F, t44580: F, t44601: F, t44609: F, t452: F, t492: F, t6305: F) -> F {
    let t44615 = -t44549 + t44552 + t44553 + t44556 + t44559 + F::new(0.1138200265427045984e0) * t2268 * t2343 * t44560 + F::new(0.1138200265427045984e0) * t2268 * t2343 * t44564 + F::new(0.1138200265427045984e0) * t6305 * t13254 + t44572 - t44574 + t44576 - t44578 + t44579 + t44580 + F::new(0.28455006635676149599e-1) * t105 * t452 * t44601 * t169 * t172 - F::new(0.37940008847568199465e-1) * t380 * t13323 - F::new(0.28455006635676149599e-1) * t105 * t492 * t44609 + F::new(0.37940008847568199465e-1) * t380 * t13299;
    t44615
}
