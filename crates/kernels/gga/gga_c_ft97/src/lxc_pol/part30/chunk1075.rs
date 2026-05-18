//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1075/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1075<F: Float>(t1882: F, t35578: F, t35710: F, t35563: F, t8392: F, t10007: F, t1091: F, t13885: F, t13927: F, t14127: F, t141873: F, t1456: F, t149800: F, t149899: F, t149950: F, t149965: F, t149967: F, t151022: F, t1901: F, t242: F, t24737: F, t2574: F, t265: F, t27742: F, t27924: F, t28128: F, t28246: F, t28276: F, t33671: F, t33728: F, t33782: F, t35353: F, t35547: F, t446: F, t6154: F, t6194: F, t6837: F, t713: F, t729: F, t773: F) -> F {
    let t151787 = t1882 * t35578;
    let t151794 = t1882 * t35710;
    let t151807 = t8392 * t35563;
    let t151827 = -F::new(2.0) / F::new(9.0) * t1901 * t10007 * t33728 * t1091 - F::new(2.0) / F::new(9.0) * t1901 * t10007 * t33782 * t1091 + F::new(2.0) / F::new(3.0) * t446 * t2574 * t773 * t35353 + F::new(2.0) / F::new(3.0) * t446 * t2574 * t265 * t151022 - F::new(2.0) * t446 * t242 * t149965 + F::new(4.0) / F::new(3.0) * t446 * t242 * t149967 - F::new(2.0) / F::new(3.0) * t446 * t729 * t13927 * t33671 - F::new(2.0) / F::new(9.0) * t151787 + F::new(2.0) / F::new(3.0) * t446 * t729 * t6154 * t28276 + t141873 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t151794 - F::new(2.0) / F::new(3.0) * t446 * t729 * t1456 * t27742 + F::new(4.0) / F::new(3.0) * t446 * t242 * t149800 - F::new(4.0) / F::new(3.0) * t1901 * t14127 * t28128 * t27924 - t151807 / F::new(27.0) - F::new(4.0) / F::new(3.0) * t1901 * t13885 * t24737 * t28246 - t446 * t729 * t35547 * t713 / F::new(3.0) - t446 * t242 * t149950 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t242 * t149899 - F::new(2.0) / F::new(3.0) * t446 * t729 * t6194 * t6837;
    t151827
}
