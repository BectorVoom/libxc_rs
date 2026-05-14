//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 759/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk759<F: Float>(t2606: F, t35566: F, t24793: F, t6917: F, t24412: F, t6930: F, t242: F, t6154: F, t6921: F, t729: F, t6861: F, t3977: F, t7502: F, t1091: F, t724: F, t7560: F) -> (F, F, F, F, F, F, F, F) {
    let t35567 = t2606 * t35566;
    let t35570 = t24793 * t6917;
    let t35573 = t24412 * t6930;
    let t35574 = t242 * t35573;
    let t35578 = t729 * t6154 * t6921;
    let t35582 = t729 * t6154 * t6861;
    let t35586 = t729 * t3977 * t7502;
    let t35590 = t724 * t7560 * t1091;
    (t35567, t35570, t35573, t35574, t35578, t35582, t35586, t35590)
}
