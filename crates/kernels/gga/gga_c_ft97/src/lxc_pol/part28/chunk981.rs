//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 981/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk981<F: Float>(t1882: F, t35107: F, t35039: F, t106623: F, t12968: F, t13140: F, t13220: F, t139675: F, t139757: F, t140087: F, t140089: F, t140094: F, t140103: F, t140112: F, t140129: F, t140338: F, t140419: F, t1901: F, t2221: F, t23443: F, t23455: F, t26520: F, t26897: F, t26928: F, t27007: F, t27334: F, t27335: F, t33034: F, t33203: F, t3446: F, t3450: F, t3455: F, t3478: F, t35063: F, t35196: F, t379: F, t63180: F, t925: F, t9419: F) -> (F,) {
    let t148055 = t1882 * t35107;
    let t148057 = t1882 * t35039;
    let t148105 = t140087 / 9.0 + t140089 / 9.0 + 2.0 / 3.0 * t148055 - 4.0 / 9.0 * t148057 + 2.0 / 3.0 * t140094 - 4.0 * t1901 * t27334 * t27335 * t26520 - 2.0 / 3.0 * t1901 * t12968 * t33034 * t3450 - 2.0 / 3.0 * t1901 * t13140 * t140419 * t3455 - 4.0 / 3.0 * t1901 * t63180 * t33203 + 2.0 / 9.0 * t1901 * t23443 * t27007 + t1901 * t9419 * t35063 / 9.0 + t1901 * t2221 * t140338 * t925 / 9.0 - 4.0 / 3.0 * t1901 * t106623 * t26928 + t1901 * t139675 * t3446 / 9.0 - 2.0 / 9.0 * t140103 - 4.0 / 9.0 * t140112 - 4.0 / 3.0 * t1901 * t13140 * t23455 * t26897 - t140129 + 2.0 * t1901 * t13140 * t139757 * t3478 - 2.0 / 9.0 * t1901 * t13220 * t35196 * t379;
    (t148105,)
}
