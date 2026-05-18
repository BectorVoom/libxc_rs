//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 988/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk988<F: Float>(t33590: F, t6745: F, t1454: F, t668: F, t33571: F, t24237: F, t35255: F, t109652: F, t6175: F, t10052: F, t1403: F, t140583: F, t140585: F, t140588: F, t140627: F, t193: F, t2347: F, t2360: F, t24231: F, t28030: F, t28036: F, t35550: F, t35639: F, t3875: F, t3886: F, t5996: F, t6002: F, t6752: F, t766: F) -> (F, F) {
    let t149769 = t6745 * t33590;
    let t149771 = t1454 * t668;
    let t149790 = t6745 * t33571;
    let t149798 = t24237 * t35255;
    let t149800 = t109652 * t6175;
    let t149802 = t140583 / F::new(54.0) - t140585 / F::new(18.0) + t149769 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t6002 * t24231 * t149771 * t3875 + F::new(2.0) / F::new(9.0) * t6002 * t28030 * t1454 * t2360 * t3886 - F::new(2.0) / F::new(27.0) * t6002 * t28036 * t1454 * t2347 * t3886 + F::new(2.0) / F::new(9.0) * t140588 - F::new(12.0) * t10052 * t35639 * t766 - t149790 / F::new(9.0) - t1403 * t193 * t140627 * t6752 / F::new(3.0) - t5996 * t35550 / F::new(3.0) - t149798 / F::new(27.0) + F::new(8.0) * t149800;
    (t149800, t149802)
}
