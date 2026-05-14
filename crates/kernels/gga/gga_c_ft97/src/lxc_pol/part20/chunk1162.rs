//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1162/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1162<F: Float>(t2567: F, t6940: F, t28205: F, t8392: F, t28209: F, t24668: F, t256: F, t10157: F, t109617: F, t109756: F, t109803: F, t1175: F, t14058: F, t14127: F, t14183: F, t14188: F, t14254: F, t1901: F, t242: F, t24507: F, t2579: F, t28299: F, t28300: F, t28349: F, t3821: F, t42575: F, t446: F, t51609: F, t53797: F, t54032: F, t6154: F, t6162: F, t6194: F, t729: F, t97957: F) -> (F,) {
    let t111016 = t2567 * t6940;
    let t111045 = 2.0 / 27.0 * t8392 * t28205;
    let t111047 = 4.0 / 27.0 * t8392 * t28209;
    let t111048 = t256 * t24668;
    let t111055 = 2.0 / 3.0 * t446 * t729 * t6154 * t14058 - 2.0 / 3.0 * t446 * t729 * t6194 * t3821 - 2.0 / 27.0 * t97957 - 4.0 / 3.0 * t1901 * t14127 * t111016 * t2579 + 4.0 / 3.0 * t446 * t242 * t109617 + 4.0 / 3.0 * t446 * t242 * t109803 - 2.0 * t446 * t10157 * t1175 * t24507 - 2.0 / 9.0 * t1901 * t42575 * t28349 - 2.0 * t1901 * t28299 * t28300 * t14254 + 2.0 / 9.0 * t1901 * t51609 * t6162 - 2.0 / 3.0 * t446 * t242 * t109756 + t111045 + t111047 + 8.0 / 9.0 * t53797 * t111048 * t14183 - 8.0 / 27.0 * t54032 * t111048 * t14188;
    (t111055,)
}
