//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 762/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk762<F: Float>(t1091: F, t33754: F, t2606: F, t7502: F, t10007: F, t265: F, t35516: F, t729: F, t1901: F, t33658: F, t33680: F, t33682: F, t35596: F, t35601: F, t35606: F, t35610: F, t35614: F, t35617: F, t446: F) -> (F, F, F, F, F, F) {
    let t35620 = t33754 * t1091;
    let t35621 = t2606 * t35620;
    let t35624 = t7502 * t1091;
    let t35625 = t10007 * t35624;
    let t35629 = t729 * t265 * t35516;
    let t35632 = t33658 + t446 * t35596 / 3.0 - 2.0 / 3.0 * t446 * t35601 - 2.0 * t446 * t35606 - 4.0 / 3.0 * t1901 * t35610 - 4.0 / 3.0 * t1901 * t35614 + 2.0 / 9.0 * t1901 * t35617 + t1901 * t35621 / 9.0 - 2.0 / 9.0 * t1901 * t35625 - t33680 + t33682 - t446 * t35629 / 3.0;
    (t35620, t35621, t35624, t35625, t35629, t35632)
}
