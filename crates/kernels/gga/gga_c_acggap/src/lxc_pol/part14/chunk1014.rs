//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1014/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1014<F: Float>(t35794: F, t4680: F, t7426: F, t8605: F, t30468: F, t4916: F, t1588: F, t7614: F, t1988: F, t8855: F, t7799: F, t8859: F) -> (F, F, F, F, F, F) {
    let t35795 = F::new(0.47172138434406228102e-2) * t35794;
    let t35797 = t7426 * t4680 * t8605;
    let t35798 = F::new(0.42874018118069736972e-3) * t35797;
    let t35799 = t30468 * t4916;
    let t35800 = F::new(0.34299214494455789578e-2) * t35799;
    let t35814 = t7614 * t1588;
    let t35816 = t1988 * t8855;
    let t35817 = F::new(0.21437009059034868486e-3) * t35816;
    let t35818 = t7799 * t8859;
    (t35795, t35798, t35800, t35814, t35817, t35818)
}
