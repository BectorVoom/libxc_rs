//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1110/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1110<F: Float>(t27028: F, t6774: F, t5329: F, t6837: F, t7794: F, t1851: F, t1856: F, t26996: F, t26955: F, t28176: F, t28190: F, t28215: F, t28925: F, t28936: F, t29123: F, t29127: F, t7772: F, t7788: F, t8091: F) -> (F, F, F, F, F, F, F, F) {
    let t29147 = t27028 * t6774;
    let t29148 = t5329 * t29147;
    let t29151 = t7794 * t6837;
    let t29152 = t5329 * t29151;
    let t29159 = t1856 * t1851;
    let t29160 = t26996 * t29159;
    let t29161 = t5329 * t29160;
    let t29170 = F::new(0.46377350260416666667e-4) * t7772 * t29127 - F::new(0.69505208333333333334e-3) * t7788 * t29148 + F::new(0.34752604166666666667e-3) * t7788 * t29152 - F::new(0.23214722222222222222e-2) * t28925 + F::new(0.30918233506944444444e-4) * t26955 * t29123 - F::new(0.23168402777777777778e-3) * t28176 - F::new(0.92754700520833333334e-4) * t7772 * t29161 - F::new(0.69505208333333333334e-3) * t7788 * t29161 - F::new(0.23168402777777777778e-3) * t28190 * t8091 - F::new(0.7722800925925925926e-4) * t28215 + F::new(0.15476481481481481481e-2) * t28936;
    (t29147, t29148, t29151, t29152, t29159, t29160, t29161, t29170)
}
