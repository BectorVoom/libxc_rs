//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 845/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk845<F: Float>(t6154: F, t6921: F, t729: F, t6861: F, t3977: F, t7502: F, t1091: F, t724: F, t7560: F, t1901: F, t33630: F, t33636: F, t35555: F, t35559: F, t35563: F, t35567: F, t35570: F, t35574: F, t446: F) -> (F, F, F, F, F) {
    let t35578 = t729 * t6154 * t6921;
    let t35582 = t729 * t6154 * t6861;
    let t35586 = t729 * t3977 * t7502;
    let t35590 = t724 * t7560 * t1091;
    let t35593 = F::new(2.0) / F::new(3.0) * t446 * t35555 - F::new(2.0) / F::new(9.0) * t1901 * t35559 + t1901 * t35563 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t35567 + F::new(2.0) / F::new(9.0) * t1901 * t35570 + F::new(4.0) / F::new(3.0) * t446 * t35574 + F::new(2.0) / F::new(3.0) * t446 * t35578 - t33630 + F::new(2.0) / F::new(3.0) * t446 * t35582 + F::new(2.0) / F::new(3.0) * t446 * t35586 + t33636 - t446 * t35590 / F::new(9.0);
    (t35578, t35582, t35586, t35590, t35593)
}
