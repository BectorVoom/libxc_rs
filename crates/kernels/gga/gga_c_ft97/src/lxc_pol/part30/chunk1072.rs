//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1072/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1072<F: Float>(t258: F, t35516: F, t35678: F, t761: F, t110010: F, t110369: F, t110751: F, t110950: F, t111330: F, t11593: F, t13839: F, t14127: F, t14163: F, t14175: F, t141784: F, t141997: F, t142347: F, t150038: F, t1901: F, t24668: F, t24789: F, t24793: F, t2599: F, t2606: F, t27767: F, t27878: F, t28129: F, t28140: F, t28157: F, t28255: F, t28360: F, t33755: F, t35566: F, t35639: F, t35697: F, t3837: F, t3842: F, t3876: F, t3880: F, t42334: F, t53927: F, t6074: F, t6075: F, t6162: F, t67996: F, t684: F, t7502: F, t9787: F) -> F {
    let t151578 = t258 * t35516;
    let t151621 = t761 * t35678;
    let t151626 = F::new(2.0) / F::new(3.0) * t1901 * t53927 * t142347 * t3880 + F::new(2.0) / F::new(9.0) * t1901 * t110369 * t6075 - F::new(4.0) / F::new(3.0) * t1901 * t110751 * t28129 + F::new(2.0) / F::new(9.0) * t1901 * t42334 * t35697 * t684 - F::new(2.0) / F::new(9.0) * t1901 * t14163 * t150038 - F::new(2.0) / F::new(9.0) * t1901 * t14175 * t35639 * t684 + t1901 * t2599 * t151578 * t684 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t9787 * t35566 + F::new(2.0) / F::new(9.0) * t1901 * t110950 * t6162 - F::new(4.0) / F::new(3.0) * t1901 * t14127 * t24668 * t28255 - F::new(2.0) / F::new(27.0) * t141784 + t1901 * t13839 * t33755 / F::new(9.0) - F::new(4.0) / F::new(9.0) * t1901 * t111330 * t27767 + F::new(4.0) * t1901 * t110010 * t7502 * t3837 + F::new(8.0) / F::new(3.0) * t1901 * t67996 * t7502 * t3842 + t1901 * t141997 * t3876 / F::new(9.0) - F::new(4.0) * t1901 * t28140 * t6074 * t27878 - F::new(4.0) / F::new(9.0) * t11593 * t24793 * t28360 - F::new(4.0) / F::new(9.0) * t11593 * t24789 * t28157 + t1901 * t2606 * t151621 * t684 / F::new(9.0);
    t151626
}
