//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1072/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1072(t258: f64, t35516: f64, t35678: f64, t761: f64, t110010: f64, t110369: f64, t110751: f64, t110950: f64, t111330: f64, t11593: f64, t13839: f64, t14127: f64, t14163: f64, t14175: f64, t141784: f64, t141997: f64, t142347: f64, t150038: f64, t1901: f64, t24668: f64, t24789: f64, t24793: f64, t2599: f64, t2606: f64, t27767: f64, t27878: f64, t28129: f64, t28140: f64, t28157: f64, t28255: f64, t28360: f64, t33755: f64, t35566: f64, t35639: f64, t35697: f64, t3837: f64, t3842: f64, t3876: f64, t3880: f64, t42334: f64, t53927: f64, t6074: f64, t6075: f64, t6162: f64, t67996: f64, t684: f64, t7502: f64, t9787: f64) -> f64 {
    let t151578 = t258 * t35516;
    let t151621 = t761 * t35678;
    let t151626 = 2.0_f64 / 3.0_f64 * t1901 * t53927 * t142347 * t3880 + 2.0_f64 / 9.0_f64 * t1901 * t110369 * t6075 - 4.0_f64 / 3.0_f64 * t1901 * t110751 * t28129 + 2.0_f64 / 9.0_f64 * t1901 * t42334 * t35697 * t684 - 2.0_f64 / 9.0_f64 * t1901 * t14163 * t150038 - 2.0_f64 / 9.0_f64 * t1901 * t14175 * t35639 * t684 + t1901 * t2599 * t151578 * t684 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t9787 * t35566 + 2.0_f64 / 9.0_f64 * t1901 * t110950 * t6162 - 4.0_f64 / 3.0_f64 * t1901 * t14127 * t24668 * t28255 - 2.0_f64 / 27.0_f64 * t141784 + t1901 * t13839 * t33755 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t1901 * t111330 * t27767 + 4.0_f64 * t1901 * t110010 * t7502 * t3837 + 8.0_f64 / 3.0_f64 * t1901 * t67996 * t7502 * t3842 + t1901 * t141997 * t3876 / 9.0_f64 - 4.0_f64 * t1901 * t28140 * t6074 * t27878 - 4.0_f64 / 9.0_f64 * t11593 * t24793 * t28360 - 4.0_f64 / 9.0_f64 * t11593 * t24789 * t28157 + t1901 * t2606 * t151621 * t684 / 9.0_f64;
    t151626
}
